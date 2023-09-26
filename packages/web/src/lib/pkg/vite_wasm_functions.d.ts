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
* @returns {Todos}
*/
export function markAllComplete(): Todos;
/**
* @returns {Todos}
*/
export function markAllActive(): Todos;
/**
* @param {string} id
* @returns {Todos}
*/
export function toggleComplete(id: string): Todos;
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
