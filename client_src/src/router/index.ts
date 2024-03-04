import { createRouter, createWebHistory, type RouteLocationNormalized } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import NotFoundView from '@/views/NotFoundView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [{
      path: '/',
      name: 'dashboard',
      meta: {
        sideMenu: true,
        title: 'Dashboard',
        order: 1
      },
      component: DashboardView
    }, {
      path: '/tickets',
      name: 'tickets',
      meta: {
        sideMenu: true,
        title: 'Tickets',
        order: 2
      },
      component: () => import('../views/TicketsView.vue'), 
      children: [{
        path: '',
        name: 'tickets.list.new',
        meta: {
          title: 'New',
        },
        component: () => import('../views/TicketsListView.vue')
      }, {
        path: '/tickets?state=my',
        name: 'tickets.list.my',
        meta: {
          title: 'My',
          showMenu: true
        },
        component: () => import('../views/TicketsListView.vue')
      }, {
        path: '/tickets?state=closed',
        name: 'tickets.list.closed',
        meta: {
          title: 'Closed',
          showMenu: true
        },
        component: () => import('../views/TicketsListView.vue')
      }, {
        path: '/tickets/:id',
        name: 'tickets.detail',
        component: () => import('../views/TicketsDetailView.vue')
      }]
    }, {
      path: '/:pathMatch(.*)*',
      component: NotFoundView
    }
  ]
})

const canAccessUI = async (to: RouteLocationNormalized) => {
  if(to.meta.allowAnonymous) return true; 
  return true;
};

router.beforeEach(async (to, from, next) => {
  const access = await canAccessUI(to);
  if(!access) return '/login';
  return next();
});
 

export default router
