<script setup lang="ts">
    import { ref } from 'vue';
    import ajax from '@/ajax';
    import type { LoginResponse } from '@/types';
    import { useRouter } from 'vue-router';
    
    const router  = useRouter(),    
        email = ref(''), 
        password = ref(''), 
        error = ref(''), 
        login = async () => {

            // await ajax.post<LoginResponse>('/api/login', {
            //     username: email.value, 
            //     password: password.value
            // }).then((data: LoginResponse) => {
            //     if(data.success) {
            //         localStorage.setItem('TICKETS_TOKEN', data.token); 
            //     } else {
            //         error.value = 'Invalid email or password';    
            //     }
            // });

            localStorage.setItem('TICKETS_TOKEN', 'debugging'); 
            router.push({
                path: '/'
            });
            location.reload();
        };
</script>

<template>
    <div class="container vertical-center">
        <div class="login-container">
            <form @submit.prevent="login">
                <div class="form-group row">
                    <label for="email" class="col-sm-3 col-form-label">Email:</label>
                    <div class="col-sm-9">
                        <input type="email" id="email" v-model="email" required  class="form-control">
                    </div>
                </div>
                <div class="form-group row">
                    <label for="password" class="col-sm-3 col-form-label">Password:</label>
                    <div class="col-sm-9">
                        <input type="password" id="password" v-model="password" required class="form-control">
                    </div>
                </div>
                <button type="submit" class="btn btn-light">
                    Login
                </button>
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
        border: $default-dark-border-size solid $default-dark-border-color;
        border-radius: 5px;
        background-color: $default-ligher-background;
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