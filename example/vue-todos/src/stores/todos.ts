import { defineStore } from 'pinia';
import * as Storage from '../services/Storage';
import type { Todo, uuid } from '../models/Todo';

function localOnly() {
  console.warn(`local storage updated but there is no DB connection`);
}

export const useStore = defineStore('todos', {
  state: () => {
    return {
      todos: [] as Todo[],
      ready: false,
      count: 'undetermined',
      dbError: undefined as string | undefined,
      dbConnectionString: ''
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
    setErrorState(err: string) {
      this.dbError = err;
    },
    setDbConnectionString(connect: string) {
      this.dbConnectionString = connect;
    },
    /**
     * Connects to DB and gets all TODOs from DB
     */
    async initializeDbBackedStore() {
      try {
        await Storage.connect();
        let count = await Storage.select_one('select count(*) as count from todos');
        console.log(`there are ${JSON.stringify(count)} TODOs in the database`);
        this.count = count;
      } catch (e) {
        this.dbError = `Failed to connect to DB: ${e}`;
        console.log(this.dbError);

        this.todos = [];
        this.ready = false;
      }

      try {
        const todos = await Storage.all();
        this.todos = todos;
        this.ready = true;
      } catch (e) {
        this.dbError = `Failure getting TODO items from DB: ${e}`;
        this.todos = [];
        this.ready = false;
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
