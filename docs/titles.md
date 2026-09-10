# Session titles

Waku names a chat after its first completed exchange.

## Behavior

A new chat displays `New task` in both the sidebar and chat tab. Waku does not derive a temporary title from the first prompt and does not wait for the provider to name its native session.

After the first successful turn produces an assistant message, Waku sends these two values to a fresh, isolated request:

- the first user message
- the complete first assistant response

The request uses the chat's provider and selected model. It asks for a plain, specific title of 2–6 words and disables agent presets and computer use. Providers that support supervised mode run the request in `Ask`; Amp keeps its required `FullAccess` mode. The isolated runtime has no resume cursor, so title generation cannot append to or alter the visible conversation.

Provider I/O runs on the background executor. A 60-second timeout bounds the request. Failure is silent and leaves the chat named `New task`.

## Eligibility

A title request starts only when all of these are true:

- the first turn completed successfully
- the turn has both a user message and a real assistant response
- the session still has the default user-owned title
- `auto_title` is empty
- no title request is already running for the session

A synthetic fallback such as a provider error or "turn completed" message does not trigger generation. A queued second turn does not cancel a title request already started from the first exchange.

## Applying the result

The provider response is trimmed, reduced to its first non-empty line, stripped of common JSON, code-fence, `Title:`, quote, and Markdown wrappers, and capped at 80 characters.

The result is stored in `AgentSession::auto_title`. `display_title()` resolves fields in this order:

1. an explicit user title
2. the generated `auto_title`
3. `New task`

Both the sidebar row and chat tab call `localized_session_title()`, which uses `display_title()`. One state update therefore refreshes both labels. The update is persisted immediately.

Provider-native `AutoTitleUpdated` events are ignored by the app. This keeps timing and title quality consistent across providers and prevents a provider's prompt-only placeholder from racing the response-aware title.

## Main files

- `src/app/streaming.rs` triggers generation after the first successful `TurnFinished` event.
- `src/app/runtime.rs` extracts the first exchange, starts the isolated provider request, normalizes the response, and applies it.
- `src/app.rs` tracks in-flight title requests.
- `src/app/sidebar.rs` renders the shared title in sidebar rows and chat tabs.
- `crates/waku-protocol/src/model.rs` owns `title`, `auto_title`, and `display_title()`.
