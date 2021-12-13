import Database from 'tauri-plugin-sql-api';
import type { QueryResult } from 'tauri-plugin-sql-api';
import { v4 } from 'uuid';
import type { Todo, uuid } from '../models/Todo';

let db: null | Database = null;

async function connect(): Promise<Database> {
  try {
    db = await Database.load('sqlite:test.db');
    console.log({ db });
    return db;
  } catch (e) {
    console.log(e);
  }
}

async function all(): Promise<Todo[]> {
  await connect();
  return await db.select('SELECT * FROM todos');
}

async function create(title: string): Promise<Todo> {
  const newTodo = {
    id: v4(),
    title,
    completed: false
  };
  if (db) {
    await db.execute('INSERT INTO todos (id, title, completed) VALUES ($1,$2,$3)', [
      newTodo.id,
      title,
      false
    ]);
  } else {
    console.warn(`There is not a valid DB connection, adding TODO to local storage only`);
  }
  return newTodo;
}

async function update(todo: Todo): Promise<Todo> {
  await db.execute('UPDATE todos SET title = $1, completed = $2 WHERE id = $3', [
    todo.title,
    todo.completed,
    todo.id
  ]);
  return todo;
}

async function remove(id: uuid): Promise<QueryResult> {
  return await db.execute('DELETE FROM todos WHERE id = $1', [id]);
}

export default {
  connect,
  all,
  create,
  update,
  remove
};
