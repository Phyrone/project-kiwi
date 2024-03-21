import type { RequestHandler } from './$types';
import { minio } from '$lib/server/minio';
import { standardizedError } from '$lib/RFC9457';
import type { UploadedObjectInfo } from 'minio';
import { json } from '@sveltejs/kit';
import { Readable } from 'stream';


export const PUT: RequestHandler = async ({ request }) => {
		const form_data = await request.formData();
		const file = form_data.get('file');

		if (file && file instanceof File) {
			const file_name = file.name;

			//@ts-ignore
			const file_upload = Readable.fromWeb(file.stream());
			console.log('stream', file_upload);
			try {
				console.log('uploading', file_name);
				const result = await new Promise<UploadedObjectInfo>((resolve, reject) => {
					minio.putObject('app2', file_name, file_upload, file.size, {
						'Content-Type': file.type
					},(error, result) => {
						if (error) {
							reject(error);
						} else {
							resolve(result);
						}
					});
				});
				console.log('upload done', result);
				return json({
					etag: result.etag,
					versionId: result.versionId
				});
			} finally {
				file_upload.destroy();
			}

		}  else {
			return standardizedError({
				type: 'comming soon',
				status: 400,
				title: 'No File Provided',
				detail: 'Upload form must include a file field.'
			});
		}
	}
;