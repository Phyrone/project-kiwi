import 'dotenv/config';
import pg, { type Sql } from 'postgres';
import rethink_db, { type Connection as RethinkConnection } from 'rethinkdb';
import { format as format_sql } from '@hokaccha/sql-formatter';

export const sql: Sql = pg('postgres://localhost/app2', {
	username: 'app2',
	password: '123456',
	debug: (connection, query, parameters) => {
		const formated_query = format_sql(query, {
			language: 'postgresql',
			keywordCase: 'upper',
			linesBetweenQueries: 1,
			params: parameters,
			indent: ''
		}).replaceAll('\n', ' ');
		console.debug(`SQL (${connection})`, `'${formated_query}'`);
	},
	connection: {
		application_name: 'project-kiwi-frontend'
	}
});

async function create_rethink_db(): Promise<RethinkConnection> {
	const connection = await rethink_db.connect({
		host: 'localhost'
		//host: '192.168.3.132',
		//password: 'rethinkdb'
	});
	connection.use('app2');
	return connection;
}

export const rethink: RethinkConnection = await create_rethink_db();

export { rethink_db as r };
