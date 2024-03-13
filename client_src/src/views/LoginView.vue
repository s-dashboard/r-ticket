<script setup lang="ts">
    import { ref } from 'vue';
    import ajax from '@/ajax';
    import type { LoginResponse } from '@/types';

    const email = ref(''), 
        password = ref(''), 
        error = ref(''), 
        login = async () => {
            await ajax.post<LoginResponse>('/api/login', {
                username: email.value, 
                password: password.value
            }).then((data: LoginResponse) => {
                if(data.success) {
                    localStorage.setItem('TICKETS_TOKEN', data.token); 
                } else {
                    error.value = 'Invalid email or password';    
                }
            });
        };
</script>

<template>
    <div class="login-container">
      <h2>Login</h2>
      <form @submit.prevent="login">
        <div class="form-group">
          <label for="email">Email:</label>
          <input type="email" id="email" v-model="email" required>
        </div>
        <div class="form-group">
          <label for="password">Password:</label>
          <input type="password" id="password" v-model="password" required>
        </div>
        <button type="submit">Login</button>
      </form>
      <div v-if="error" class="error">{{ error }}</div>
    </div>
  </template>
  
  <style scoped lang="scss">
    .login-container {
        max-width: 400px;
        margin: 0 auto;
        padding: 20px;
        border: 1px solid #ccc;
        border-radius: 5px;
    }
    
    .form-group {
        margin-bottom: 20px;
    }
    
    label {
        display: block;
        margin-bottom: 5px;
    }
    
    input[type="email"],
    input[type="password"] {
        width: 100%;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 5px;
    }
    
    button {
        padding: 10px 20px;
        background-color: #007bff;
        color: #fff;
        border: none;
        border-radius: 5px;
        cursor: pointer;
    }
    
    .error {
        color: red;
        margin-top: 10px;
    }
  </style>