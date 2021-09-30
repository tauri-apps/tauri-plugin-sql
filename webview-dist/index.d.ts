export interface QueryResult {
    rowsAffected: number;
    lastInsertId: number;
}
export default class Database {
    path: string;
    constructor(path: string);
    static load(path: string): Promise<Database>;
    static get(path: string): Database;
    execute(query: string, bindValues?: any[]): Promise<QueryResult>;
    select<T>(query: string, bindValues?: any[]): Promise<T>;
}
