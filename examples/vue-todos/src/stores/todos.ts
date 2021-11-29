import { defineStore } from 'pinia';
import Storage from '../services/Storage';
import type { Todo, uuid } from '../models/Todo';

function localOnly() {
  console.warn(`local storage updated but there is no DB connection`);
}

export const useStore = defineStore('todos', {
  state: () => {
    return {
      todos: [] as Todo[],
      ready: false,
      dbError: undefined as string | undefined
    };
  },
  getters: {
    all(state) {
      return state.todos as Todo[];
    },
    completed(state) {
      return state.todos.filter((todo: Todo) => todo.completed) as Todo[];
    },
    incomplete(state) {
      return state.todos.filter((todo: Todo) => !todo.completed) as Todo[];
    }
  },
  actions: {
    /**
     * Connects to DB and gets all TODOs from DB
     */
    async init() {
      const connection = await Storage.connect();
      if (connection.db) {
        const todos = await Storage.all();
        console.log({ db: connection.db });

        this.todos = todos;
        this.ready = true;
      } else {
        this.todos = [];
        this.ready = false;
        console.warn(`Failed to connect to DB: ${connection.error}`);
        this.dbError = connection.error;
      }
    },
    async add(task: string) {
      const newTodo: Todo = await Storage.create(task);
      this.todos.push(newTodo);
    },
    async markFinished(todo: Todo) {
      const updated = { ...todo, completed: true };
      try {
        if (this.ready) {
          await Storage.update(updated);
        } else {
          localOnly();
        }
        this.todos = this.todos.map((i: Todo) => (i.id === updated.id ? updated : i));
      } catch (e) {
        localOnly();
      }
    },

    async markIncomplete(todo: Todo) {
      const updated = { ...todo, completed: false };
      if (this.ready) {
        await Storage.update(updated);
      } else {
        localOnly();
      }
      this.todos = this.todos.map((i: Todo) => (i.id === todo.id ? updated : i));
    },
    async remove(id: uuid) {
      if (this.ready) {
        await Storage.remove(id);
      } else {
        localOnly();
      }
      this.todos = this.todos.filter((i: Todo) => i.id !== id);
    },
    setDbError(err: string) {
      this.dbError = err;
    }
  }
});
