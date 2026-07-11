import importlib.util
import unittest
from datetime import datetime, timezone
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("eclock_esaa_state.py")
SPEC = importlib.util.spec_from_file_location("eclock_esaa_state", MODULE_PATH)
state = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
SPEC.loader.exec_module(state)


def event(seq, action, task_id="T-1", decision=None, ts="2026-07-11T10:00:00Z"):
    payload = {"task_id": task_id}
    if decision:
        payload["decision"] = decision
    return {
        "event_seq": seq,
        "event_id": f"EV-{seq:08d}",
        "action": action,
        "actor": "agent-impl",
        "ts": ts,
        "payload": payload,
    }


class StateMachineTests(unittest.TestCase):
    def test_all_forward_transitions(self):
        events = [
            event(1, "task.create"),
            event(2, "claim"),
            event(3, "complete"),
            event(4, "review", decision="approve"),
        ]
        expected = ["todo", "in_progress", "review", "done"]
        for size, status in enumerate(expected, start=1):
            self.assertEqual(state.derive_task(events[:size])[0], status)

    def test_request_changes_returns_to_in_progress(self):
        events = [event(1, "claim"), event(2, "complete"), event(3, "review", decision="request_changes")]
        self.assertEqual(state.derive_task(events)[0], "in_progress")

    def test_no_events_defaults_to_todo_and_dependency_blocks(self):
        tasks = [
            {"task_id": "T-1", "title": "base"},
            {"task_id": "T-2", "title": "dependent", "depends_on": ["T-1"]},
        ]
        snapshot = state.derive_snapshot(tasks, [])
        self.assertEqual(snapshot[0]["status"], "todo")
        self.assertTrue(snapshot[1]["blocked"])

    def test_elapsed_time(self):
        now = datetime(2026, 7, 11, 11, 2, 3, tzinfo=timezone.utc)
        self.assertEqual(state.elapsed("2026-07-11T10:00:00Z", now), "01:02:03")
        self.assertEqual(state.elapsed("invalid", now), "--:--:--")

    def test_operational_panel_groups_tasks_and_omits_ledger_details(self):
        now = datetime(2026, 7, 11, 11, 2, 3, tzinfo=timezone.utc)
        tasks = [
            {"task_id": "T-TODO", "title": "Aguardando"},
            {"task_id": "T-IMPL", "title": "Implementando", "assigned_to": "agent-impl"},
            {"task_id": "T-QA", "title": "Revisando", "assigned_to": "agent-qa"},
            {"task_id": "T-DONE", "title": "Concluída"},
        ]
        events = [
            event(1, "task.create", "T-TODO"),
            event(2, "task.create", "T-IMPL"),
            event(3, "claim", "T-IMPL"),
            event(4, "task.create", "T-QA"),
            event(5, "claim", "T-QA"),
            event(6, "complete", "T-QA"),
            event(7, "task.create", "T-DONE"),
            event(8, "claim", "T-DONE"),
            event(9, "complete", "T-DONE"),
            event(10, "review", "T-DONE", decision="approve"),
        ]
        output = state.render_snapshot(state.derive_snapshot(tasks, events), now)
        self.assertIn("status T:1 I:1 R:1 D:1 B:0", output)
        self.assertIn("I: T-IMPL · 01:02:03 · agent-impl", output)
        self.assertIn("Implementando", output)
        self.assertIn("R: T-QA · 01:02:03 · agent-qa", output)
        self.assertIn("Revisando", output)
        self.assertNotIn("último", output.lower())
        self.assertNotIn("event_id", output.lower())
        self.assertNotIn("EV-", output)

    def test_empty_active_sections_are_explicit(self):
        output = state.render_snapshot(
            [{"task_id": "T-1", "title": "Done", "status": "done", "blocked": False}],
        )
        self.assertIn("I: nenhuma tarefa", output)
        self.assertIn("R: nenhuma tarefa", output)


if __name__ == "__main__":
    unittest.main()
