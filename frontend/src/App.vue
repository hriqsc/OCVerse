<script setup lang="ts">
import LoginModal from '@/components/auth/LoginModal.vue'
</script>

<template>
  <router-view />
  <!-- Montado uma única vez: qualquer página chama useAuthModal().openLogin() para exibi-lo -->
  <LoginModal />
</template>
