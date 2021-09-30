import Database from 'tauri-plugin-sql-api';
import type { Todo } from '../models/Todo';

let db = null
const load = Database.load('sqlite:test.db').then(instance => {
  db = instance
  return db
})

async function all(): Promise<Todo[]> {
  await load
  return await db.select('SELECT * FROM todos')
}

async function create(title: string): Promise<Todo> {
  const { lastInsertId: id } = await db.execute('INSERT INTO todos (title) VALUES ($1)', [title]);
  return {
    id,
    title,
    completed: false,
  };
}

async function update(todo: Todo): Promise<Todo> {
  await db.execute('UPDATE todos SET title = $1, completed = $2 WHERE id = $3', [todo.title, todo.completed, todo.id]);
  return todo;
}

async function remove(id: number): Promise<boolean> {
  return await db.execute('DELETE FROM todos WHERE id = $1', [id]);
}

export default {
  all,
  create,
  update,
  remove
};
