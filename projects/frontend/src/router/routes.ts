import type { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', name: 'home', component: () => import('pages/IndexPage.vue') },
      { path: 'learn', name: 'learn', component: () => import('pages/LearnPage.vue') },
      { path: 'contact-us', name: 'contact-us', component: () => import('pages/ContactPage.vue') },
      { path: 'prizes', name: 'prizes', component: () => import('pages/PrizePage.vue') },
      { path: 'login', name: 'login', component: () => import('pages/LoginPage.vue') },
      { path: 'forgot-login', name: 'forgot-login', component: () => import('pages/ForgotLoginPage.vue') },
      { path: 'logout', component: () => import('pages/LogoutPage.vue'), name: 'logout' },
    ],
  },
  {
    path: '/secure',
    component: () => import('layouts/PlaneLayout.vue'),
    children: [
      { path: '/', component: () => import('pages/secure/ScoreBoardPage.vue'), name: 'scoreboard' },
      { path: 'active-tips/:tournamentId?', component: () => import('pages/secure/TipsPage.vue'), name: 'active-tips' },
      { path: 'manage-tip/:tournamentId/:tipStrategyId/:id?', component: () => import('pages/secure/TipManagePage.vue'), props: true, name: 'manage-tip' },
      {
        path: 'past-tips/:tournamentId?',
        component: () => import('pages/secure/PastTipsPage.vue'),
        name: 'past-tips',
      },
      { path: 'chat', component: () => import('pages/secure/ChatPage.vue'), name: 'chat' },
      { path: 'account', component: () => import('pages/secure/AccountPage.vue'), name: 'account' },
    ],
  },
  {
    path: '/admin',
    component: () => import('layouts/AdminLayout.vue'),
    children: [
      { path: '', component: () => import('pages/admin/AdminStatsPage.vue'), name: 'admin-stats' },
      {
        path: 'notifications',
        component: () => import('pages/admin/AdminNotificationPage.vue'),
        name: 'admin-notifications',
      },
      {
        path: 'users',
        component: () => import('pages/admin/AdminUsersManagerPage.vue'),
        name: 'admin-users',
      },
      {
        path: 'tips',
        component: () => import('pages/admin/AdminTipsPage.vue'),
        name: 'admin-tips',
      },
      {
        path: 'completed-tips',
        component: () => import('pages/admin/AdminComplatedTipsPage.vue'),
        name: 'admin-completed-tips',
      },
      {
        path: 'manage/tournament',
        component: () => import('pages/admin/manage/TournamentDashboardPage.vue'),
        name: 'admin-manage-tournament',
      },
      {
        path: 'manage/new-tournament',
        component: () => import('pages/admin/manage/ManageTournamentPage.vue'),
        name: 'admin-manage-new-tournament',
      },
      {
        path: 'manage/edit-tournament/:id',
        component: () => import('pages/admin/manage/ManageTournamentPage.vue'),
        name: 'admin-manage-edit-tournament',
      },
      {
        path: 'manage/tournament/:tournamentId',
        children: [
          {
            path: 'groups',
            component: () => import('pages/admin/manage/GroupDashboardPage.vue'),
            name: 'groups-dashboard',
          },
          {
            path: 'groups/manage/:id?',
            component: () => import('pages/admin/manage/ManageGroupPage.vue'),
            props: true,
            name: 'manage-group'
          },
          {
            path: 'country-groups',
            component: () => import('pages/admin/manage/CountryGroupDashboardPage.vue'),
            props: true,
            name: 'country-groups-dashboard'
          },
          {
            path: 'country-groups/manage/:id?',
            component: () => import('pages/admin/manage/ManageCountryGroupPage.vue'),
            props: true,
            name: 'manage-country-group'
          },
          {
            path: 'games',
            component: () => import('pages/admin/manage/GameDashboardPage.vue'),
            props: true,
            name: 'games-dashboard'
          },
          {
            path: 'games/manage/:id?',
            component: () => import('pages/admin/manage/ManageGamePage.vue'),
            props: true,
            name: 'manage-game'
          },
          {
            path: 'strategies',
            component: () => import('pages/admin/manage/StrategyDashboardPage.vue'),
            props: true,
            name: 'strategies-dashboard'
          },
          {
            path: 'strategies/manage/:id?',
            component: () => import('pages/admin/manage/ManageStrategyPage.vue'),
            props: true,
            name: 'manage-strategy'
          }
        ],
      }
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
