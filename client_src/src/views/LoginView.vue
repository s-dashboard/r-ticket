<script setup lang="ts">
    import { ref } from 'vue';
    import ajax from '@/ajax';
    import type { LoginResponse } from '@/types';
    import { useRouter } from 'vue-router';
    
    const router  = useRouter(),    
        username = ref(''), 
        password = ref(''), 
        error = ref(''), 
        login = async () => {

            await ajax.post<LoginResponse>('/api/auth/', {
                username: username.value, 
                password: password.value
            }).then((data: LoginResponse) => {

                localStorage.setItem('TICKETS_TOKEN', data.token_value); 
                router.push({
                    path: '/'
                });

                location.reload();
            }).catch(() => {
                error.value = 'Invalid email or password';                    
            });
        };
</script>

<template>
    <div class="container vertical-center">
        <div class="login-container">
            <div class="login-container-image"></div>
            <form @submit.prevent="login">
                <div class="form-group row">
                    <label for="username" class="col-sm-3 col-form-label">Email/Username:</label>
                    <div class="col-sm-9">
                        <input id="username" v-model="username" required  class="form-control">
                    </div>
                </div>
                <div class="form-group row">
                    <label for="password" class="col-sm-3 col-form-label">Password:</label>
                    <div class="col-sm-9">
                        <input type="password" id="password" v-model="password" required class="form-control">
                    </div>
                </div>
                <div class="row">
                    <div class="col-auto">
                        <button type="submit" class="btn btn-light">
                            Login
                        </button>
                    </div>
                    <div class="col-auto me-auto p-1">| <a href="">Forgot password?</a></div>
                </div>
            </form>
            <div v-if="error" class="error">{{ error }}</div>
        </div>
    </div>
</template>
  
  <style scoped lang="scss">
    .login-container {
        width: 400px;
        margin: 0 auto;
        padding: 20px;

        &-image {
            margin: 0 auto 2em auto; 
            height: 200px;
            width: 200px;
            border-radius: 100px;
            background-color: blue;
        }
    }
    
    .form-group {
        margin-bottom: 20px;
    }
    
    label {
        display: block;
        margin-bottom: 5px;
    }
    
    .error {
        color: red;
        margin-top: 10px;
    }

    .vertical-center {
        min-height: 100%;
        /* Fallback for browsers do NOT support vh unit */
        min-height: 100vh;
        /* These two lines are counted as one :-)       */

        display: flex;
        align-items: center;
    }
  
</style>