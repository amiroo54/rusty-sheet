import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: () => import(/* webpackChunkName: "about" */ '../views/HomeView.vue')
  },
  {
    path: '/create',
    name: "character creation",
    component: () => import('../views/CharacterCreationView.vue')
  },
  {
    path: '/view',
    name: "character view",
    component: () => import('../views/CharacterViewView.vue')
  },
  {
    path: '/list',
    name: "character list",
    component: () => import('../views/CharacterListView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
})

export default router