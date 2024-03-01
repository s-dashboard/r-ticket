import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import NotFoundView from '@/views/NotFoundView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      meta: {
        sideMenu: true,
        title: 'Dashboard'
      },
      component: DashboardView
    }, {
      path: '/tickets',
      name: 'tickets',
      meta: {
        sideMenu: true,
        title: 'Tickets'
      },
      component: () => import('../views/TicketsView.vue'), 
      children: [{
        path: '/tickets/:id', 
        name: 'tickets-detail',
        component: () => import('../views/TicketsDetailView.vue')
      }]
    },
    {
      path: '/:pathMatch(.*)*',
      component: NotFoundView
    }
  ]
})

export default router
