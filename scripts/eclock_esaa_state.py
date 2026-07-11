#!/usr/bin/env python3
"""Read-only ESAA state-machine snapshot for the eclock widget."""

from __future__ import annotations

import json
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable

def review_decision(event: dict[str, Any]) -> str | None:
    payload = event.get("payload") or {}
    return payload.get("decision") or (payload.get("payload") or {}).get("decision")


def status_from_event(event: dict[str, Any]) -> str | None:
    action = event.get("action")
    if action == "task.create":
        return "todo"
    if action == "claim":
        return "in_progress"
    if action == "complete":
        return "review"
    if action == "review":
        return "in_progress" if review_decision(event) == "request_changes" else "done"
    return None


def derive_task(events: Iterable[dict[str, Any]]) -> tuple[str, dict[str, Any] | None]:
    status = "todo"
    last_transition = None
    for event in sorted(events, key=lambda item: item.get("event_seq", 0)):
        next_status = status_from_event(event)
        if next_status is not None:
            status = next_status
            last_transition = event
    return status, last_transition


def elapsed(started_at: str | None, now: datetime | None = None) -> str:
    if not started_at:
        return "--:--:--"
    try:
        start = datetime.fromisoformat(started_at.replace("Z", "+00:00"))
    except ValueError:
        return "--:--:--"
    current = now or datetime.now(timezone.utc)
    seconds = max(0, int((current - start).total_seconds()))
    hours, remainder = divmod(seconds, 3600)
    minutes, seconds = divmod(remainder, 60)
    return f"{hours:02d}:{minutes:02d}:{seconds:02d}"


def derive_snapshot(tasks: list[dict[str, Any]], events: list[dict[str, Any]]) -> list[dict[str, Any]]:
    events_by_task: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for event in events:
        payload = event.get("payload") or {}
        task_id = payload.get("task_id")
        if task_id:
            events_by_task[str(task_id)].append(event)

    snapshot = []
    statuses = {}
    for task in tasks:
        task_id = str(task.get("task_id") or task.get("taskId") or "")
        status, last = derive_task(events_by_task.get(task_id, []))
        statuses[task_id] = status
        snapshot.append(
            {
                "task_id": task_id,
                "title": task.get("title") or "",
                "status": status,
                "assigned_to": task.get("assigned_to") or "",
                "last_transition": last,
                "depends_on": task.get("depends_on") or task.get("dependsOn") or [],
            }
        )

    for item in snapshot:
        item["blocked"] = any(statuses.get(dep, "todo") != "done" for dep in item["depends_on"])
    return snapshot


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]


def actor_name(event: dict[str, Any] | None) -> str:
    if not event:
        return "-"
    actor = event.get("actor")
    return str(actor.get("id") if isinstance(actor, dict) else actor or "-")


def render_snapshot(snapshot: list[dict[str, Any]], now: datetime | None = None) -> str:
    counts = Counter(item["status"] for item in snapshot)
    blocked = sum(bool(item["blocked"]) for item in snapshot)
    lines = [
        f"status T:{counts['todo']} I:{counts['in_progress']} "
        f"R:{counts['review']} D:{counts['done']} B:{blocked}",
    ]

    for status, label in (("in_progress", "I"), ("review", "R")):
        tasks = [item for item in snapshot if item["status"] == status]
        if not tasks:
            lines.append(f"{label}: nenhuma tarefa")
            continue
        for item in tasks:
            event = item["last_transition"] or {}
            assigned = item["assigned_to"] or actor_name(event)
            timer = elapsed(event.get("ts"), now)
            blocked_hint = " · bloqueada" if item["blocked"] else ""
            lines.append(
                f"{label}: {item['task_id']} · {timer} · {assigned}{blocked_hint} · "
                f"{item['title'][:32]}"
            )
    return "\n".join(lines)


def render(root: Path, now: datetime | None = None) -> str:
    roadmap = load_json(root / ".roadmap" / "roadmap.json")
    events = load_jsonl(root / ".roadmap" / "activity.jsonl")
    snapshot = derive_snapshot(roadmap.get("tasks") or [], events)
    return render_snapshot(snapshot, now)


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print("uso: eclock_esaa_state.py ROOT", file=sys.stderr)
        return 2
    try:
        print(render(Path(argv[1])))
    except (OSError, json.JSONDecodeError, TypeError) as error:
        print(f"estados: falha ao ler ledger ({error})")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
