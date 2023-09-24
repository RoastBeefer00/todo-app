/* tslint:disable */
/* eslint-disable */
/**
* @returns {Promise<any>}
*/
export function getTodos(): Promise<any>;
/**
* @param {string} task
* @returns {Todos}
*/
export function addTodo(task: string): Todos;
/**
* @param {string} id
* @returns {Todos}
*/
export function deleteTodo(id: string): Todos;
/**
*/
export function init(): void;
/**
*/
export class Todo {
  free(): void;
}
/**
*/
export class Todos {
  free(): void;
}
