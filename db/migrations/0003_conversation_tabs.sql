ALTER TABLE `sessions` ADD `conversation_root_id` text;
--> statement-breakpoint
CREATE INDEX `sessions_by_conversation_root` ON `sessions` (`conversation_root_id`,`updated_at`);
