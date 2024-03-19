import type {Handle} from '@sveltejs/kit';

import {type ExtendedRFC9457Error} from '$lib/RFC9457';

export const handle: Handle = async ({event, resolve}) => {
    //TODO session
    return resolve(event);
};

export const handleError = async ({status, message, error, event}) => {

    const response = errorMessageByCode(status, message, event.url.pathname);
    console.log('response', response);
    if (response  ) {
        console.error('unhandled error', error);
        event.setHeaders({
            'Content-Type': 'application/problem+json',
            'Accept': 'application/json, application/problem+json',
            'Content-Language': 'en'
        });
        // @ts-ignore
        return response;
    }else {
        return;
    }
};

function errorMessageByCode(
    code: number,
    message: string,
    path: string,
): ExtendedRFC9457Error  | undefined {
    let part: {
        type: string;
        title: string;
    };
    switch (true) {
        case (code >= 500):
            part = {
                type: 'https://www.rfc-editor.org/rfc/rfc9110.html#name-500-internal-server-error',
                title: 'Internal Server Error'
            };
            break;
        case (code == 400):
            part = {
                type: 'https://www.rfc-editor.org/rfc/rfc9110.html#name-400-bad-request',
                title: 'Bad Request'
            };
            break;

        default:
            return undefined;
    }

    return {
        success: false,
        detail: message,
        instance: path,
        ...part
    };
}
