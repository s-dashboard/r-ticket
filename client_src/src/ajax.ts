interface Ajax {
    post<T>(url: string, data: any): Promise<T>;
    put<T>(url: string, data: any): Promise<T>;
    delete<T>(url: string, data: any): Promise<T>;
    get<T>(url: string, params: any): Promise<T>;
}

const ajax: Ajax = {
    post: async (url: string = '', data: any = {}): Promise<any> => {
        return (await fetch(url, {
            method: "POST",
            mode: "cors",
            cache: "no-cache",
            credentials: "same-origin",
            headers: {
                "Content-Type": "application/json"
            },
            redirect: "follow",
            referrerPolicy: "no-referrer",
            body: JSON.stringify(data)
        })).json();
    },

    put: async (url: string = '', data: any = {}): Promise<any> => {
        return (await fetch(url, {
            method: "PUT",
            mode: "cors",
            cache: "no-cache",
            credentials: "same-origin",
            headers: {
                "Content-Type": "application/json"
            },
            redirect: "follow",
            referrerPolicy: "no-referrer",
            body: JSON.stringify(data)
        })).json();
    },

    delete: async (url: string = '', data: any = {}): Promise<any> => {
        return (await fetch(url, {
            method: "DELETE",
            mode: "cors",
            cache: "no-cache",
            credentials: "same-origin",
            headers: {
                "Content-Type": "application/json"
            },
            redirect: "follow",
            referrerPolicy: "no-referrer",
            body: JSON.stringify(data)
        })).json();
    },

    get: async (url: string = '', params: any = {}): Promise<any> => {
        const queryStringObj = new URLSearchParams(params), 
            queryString = queryStringObj.toString(); 

        let urlWithParams: string = url; 
        if(queryString !== '') {
            urlWithParams += '?' + queryString; 
        }
        return (await fetch(urlWithParams)).json();
    }
}; 

export default ajax; 