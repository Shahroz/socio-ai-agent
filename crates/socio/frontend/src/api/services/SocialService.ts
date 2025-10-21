/* generated using openapi-typescript-codegen -- do no edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ConfigResponse } from '../models/ConfigResponse';
import type { SocialMediaConfig } from '../models/SocialMediaConfig';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class SocialService {

    /**
     * Configure social media platform API credentials
     * @returns ConfigResponse Configuration successful
     * @throws ApiError
     */
    public static handleSocialConfig({
        requestBody,
    }: {
        requestBody: SocialMediaConfig,
    }): CancelablePromise<ConfigResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/social/config',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request`,
                500: `Internal server error`,
            },
        });
    }

}
