/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class WebsocketService {

    /**
     * WebSocket connection endpoint
     * @returns void
     * @throws ApiError
     */
    public static handleWebsocket(): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/ws',
            errors: {
                400: `Bad request`,
            },
        });
    }

}
