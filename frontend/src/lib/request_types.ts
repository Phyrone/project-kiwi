import type { AuthenticationResponseJSON } from '@simplewebauthn/types';

export type LoginRequestUserPass = {
	user: string;
	password: string;
	remember?: boolean;
};

export type LoginRequestWebauthn = {
	authn: AuthenticationResponseJSON;
};

export type LoginUserResponse = {
	status: 'ok' | 'invalid';
};

export type PostCreateRequest = {
	title: string;
	content?: string;
};

export type SendMessageRequest = {
	reply_to?: string;
	message: string;
};

export type SendMessageResponse = {
	id: string;
};

export type GetRequestSearchParams = {
	since?: string;
	until?: string;
	limit?: number;
	offset?: number;
	ids?: string[];
	users?: string[];
	watch?: boolean;
};

export type RegisterRequest = {
	user: string;
	password?: string;
};

export type LiveSearchParams = {
	intents?: string[];
};

export type UploadLinkRequest = {
	file_name: string;
	ttl?: string | number;
}

export type GatewayDataResponse = {
	link: string;
	token?: string;

}