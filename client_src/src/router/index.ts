import { createRouter, createWebHistory, type RouteLocationNormalized } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import NotFoundView from '@/views/NotFoundView.vue'
import LoginView from '@/views/LoginView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [{
      path: '/',
      name: 'dashboard',
      meta: {
        topMenu: true,
        title: 'Dashboard',
        iconCls: 'fa-bolt',
        order: 1
      },
      component: DashboardView
    }, {
      path: '/tickets',
      name: 'tickets',
      meta: {
        topMenu: true,
        title: 'Issues',
        iconCls: 'fa-inbox',
        order: 2
      },
      component: () => import('../views/TicketsView.vue'), 
      children: [{
        path: '',
        name: 'tickets.list',
        component: () => import('../views/TicketsListView.vue')
      }, {
        path: '/tickets/:id',
        name: 'tickets.detail',
        component: () => import('../views/TicketsDetailView.vue')
      }]
    },{
      path: '/login',
      component: LoginView
    },{
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
