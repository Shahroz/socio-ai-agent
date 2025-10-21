/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { ChatMessage } from './ChatMessage';

/**
 * Chat response payload
 */
export type ChatResponse = {
    message: ChatMessage;
    /**
     * Session ID for the conversation
     */
    session_id: string;
};

