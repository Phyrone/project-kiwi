import pg, { type Sql } from 'postgres';
import rethink_db, { type Connection as RethinkConnection } from 'rethinkdb';

export const sql: Sql = pg('postgres://localhost/app2', {
	username: 'app2',
	password: '123456',
	debug: (connection, query, parameters) => {
		if (parameters.length > 0) console.debug('SQL', "'" + query + "'", 'with', parameters);
		else console.debug('SQL', "'" + query + "'");
	},
	connection: {
		application_name: 'project-kiwi-frontend'
	}
});

async function create_rethink_db(): Promise<RethinkConnection> {
	const connection = await rethink_db.connect({
		host: '192.168.3.132',
		password: 'rethinkdb'
	});
	connection.use('app2');
	return connection;
}

export const rethink: RethinkConnection = await create_rethink_db();

export { rethink_db as r };
