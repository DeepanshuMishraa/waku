ALTER TABLE `sessions` ADD `chat_status` text;
--> statement-breakpoint
UPDATE `sessions` SET `chat_status` = COALESCE(
    (SELECT json_extract(data, '$.chat_status') FROM session_details WHERE session_details.session_id = sessions.id),
    'in-progress'
);
