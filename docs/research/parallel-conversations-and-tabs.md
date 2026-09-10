# Parallel conversations and tabs

## Summary

The products separate three concepts: (1) a visible tab/navigation item, (2) a durable agent conversation/session, and (3) an isolated filesystem workspace, usually a Git worktree. Conductor and Superset make the worktree the primary parallelism boundary; Waku makes the session the primary durable object and already supports provider-history forking.

## Findings

1. **Conductor is workspace-first.** A workspace owns a Git worktree/branch, files, processes, terminal, chat, diff, checks, and PR flow. Multiple agent tabs may share one workspace; separate workspaces are recommended when agents need different branches/files. Sources: [workspaces and branches](https://www.conductor.build/docs/concepts/workspaces-and-branches), [Git worktrees](https://www.conductor.build/docs/concepts/git-worktrees), [parallel Codex sessions](https://www.conductor.build/docs/guides/parallel-agents/run-multiple-codex-sessions).

2. **Superset is also workspace-first.** A workspace owns a branch, working directory, terminals, and ports. Its terminal tabs and splits are presentation/process state, not conversation branches. A projectless session is a special managed scratch workspace. Source: [Superset workspaces](https://github.com/superset-sh/superset/blob/main/apps/docs/content/docs/workspaces.mdx), [terminal integration](https://github.com/superset-sh/superset/blob/main/apps/docs/content/docs/terminal-integration.mdx).

3. **T3 Code is thread/session-centric, with workspace attachment.** Threads contain conversation/work history and provider-session state; worktrees can be attached for isolated editing. Its issues/plans distinguish thread identity from pane/tab management. Source: [T3 Code repository](https://github.com/pingdotgg/t3code), [thread sidebar](https://github.com/pingdotgg/t3code/blob/main/docs/user/thread-sidebar.md), [worktree design](https://github.com/pingdotgg/t3code/blob/main/.plans/git-integration-branch-picker-worktrees.md).

4. **Conversation forks are not worktree forks.** OpenCode exposes a native session fork; session branching copies context/history but does not itself guarantee filesystem isolation. Use a new worktree when both branches may edit. Sources: [OpenCode server API](https://opencode.ai/docs/server/), [VS Code session management](https://code.visualstudio.com/docs/agents/run/sessions/manage-sessions), [Git worktree](https://git-scm.com/docs/git-worktree).

5. **Waku already separates the axes.** `AgentSession` is persisted by UUID and owns title, project, provider cursor, transcript, turns, queue, and status. `SessionWorkspace` separately represents `Local`, draft `NewWorktree`, or materialized `Worktree`. Current top-level chat tabs are only a list of session UUIDs (`main_chat_tabs`), but the plus action calls `create_new_chat_tab`, which creates and pushes a normal global `AgentSession`; therefore it appears in the sidebar by design. Sources: local `src/app/sessions.rs`, `src/app/sidebar.rs`, `crates/waku-protocol/src/model.rs`.

6. **Waku already has conversation branching.** Response forking calls provider-specific native fork APIs, creates a new `AgentSession` through `fork_through_turn`, remaps provider IDs/checkpoint refs, and copies Git checkpoint refs. This is distinct from creating a worktree. Source: local `src/app/runtime.rs`, `crates/waku-protocol/src/model.rs`.

## Recommended action plan

1. **Define the model explicitly:** `Session` = conversation/history; `Workspace` = checkout/worktree; `View tab` = window-local way to switch sessions or tool surfaces. Opening a tab must not imply filesystem isolation.

2. **Keep the sidebar as the durable session catalog.** Add a lightweight window-local open-session tab strip. Its state should contain session IDs, order, active tab, and perhaps pinned state; it should not own or duplicate transcripts.

3. **Change the header plus behavior to create a session in the current conversation context, not a new sidebar task.** The likely minimal implementation is a view-level child/branch relation or tab-local conversation object, rather than `state.new_session` immediately adding a root-level session. Preserve a durable ID if the child must survive restart, but store lineage/parent and tab placement separately from the sidebar’s root grouping.

4. **Make workspace policy explicit at creation.** Default “new conversation in this chat” to the current workspace only for read/discussion/coordination. Warn before concurrent editing in a shared local checkout. Offer “new isolated worktree” for independent implementation work, materialized on first submission as Waku does today.

5. **Reuse existing provider fork infrastructure where the desired behavior is a branch of the current conversation.** A new header conversation should either start empty with inherited project/workspace settings, or fork provider history from a selected point; these are different commands and should not be conflated. Keep provider RPC, checkpoint copying, worktree creation, and hydration off the UI thread.

6. **Add lineage and lifecycle tests before UI work:** header-plus does not add an unintended sidebar root item; tab switching preserves each composer/transcript; closing a tab does not delete its session; restart restores open-tab state; shared-workspace warnings appear; isolated-worktree choice creates separate workspace state; simultaneous runtimes remain keyed by session ID.

7. **Later consider sidebar hierarchy:** project → workspace/branch → sessions, with date/updated ordering as an alternate view. This borrows the useful isolation model from Conductor/Superset without replacing Waku’s session-centric persistence.

## Caveats

Conductor is commercial, so implementation details are not public. T3’s pane/tab behavior is partly documented through issues/plans and should be checked against the current release before copying terminology. Local Waku source is authoritative if it differs from external snapshots.
