import { toValue, type App } from "vue";

type OnSaveAction = (form: any) => void;
type OnValidate = (value: string) => boolean;

type FieldValidator = {
    [key: string]: OnValidate
};


const getInputElement = (el: Element): Element => {
    const name: string = el.getAttribute('data-name') || '', 
        elType: string = el.getAttribute('data-type')?.toLowerCase() || 'text'; 
    
    let inputEl: Element | null = null;

    switch(elType) {
        case 'number': 
        case 'text': 
            inputEl = document.createElement('input'); 
        break;
        case 'textarea':
            inputEl = document.createElement('textarea');
        break;
        default: 
            throw Error('Unsupported type ' + elType);
    }

    inputEl.setAttribute('name', name);
    
    return inputEl;
}


const initClickable = (options: any) => {
    const cssClass = options.cssClass || 'clickable-field', 
    inputDefaultCssClass = options.inputDefaultCssClass || 'form-control',
    errorCssClass = options.errorCssClass || 'error-message';

    return (form: any, sourceValues: any, onSave: OnSaveAction, validators?: FieldValidator) => {

        const fields: Element[] = Array.from(document.getElementsByClassName(cssClass));

        fields.forEach((item: Element) => {
            const clickableField = (e: any) => {
                const input = document.createElement('input'),
                    element = e.target;

                if (element.classList.contains(cssClass)) {
                    input.name = element.getAttribute('data-name') || '';
                    const validator = validators ? validators[input.name] : null;
                    input.value = sourceValues !== null ? sourceValues[input.name] : '';
                    input.type = 'text';
                    input.classList.add(inputDefaultCssClass);

                    input.addEventListener('blur', () => {
                        if(!validator || validator && validator(input.value)) {
                            onSave(form);
                            sourceValues[input.name] = input.value;
                            item.innerHTML = input.value;
                            item.addEventListener('click', clickableField, { once: true });
                        }
                    })

                    const firstNode = element.childNodes[0];
                    if (firstNode !== null && firstNode !== undefined) {
                        element.removeChild(firstNode);
                    }

                    element.appendChild(input);
                    input.focus();
                }
            };

            item.addEventListener('click', clickableField, { once: true });
        });
    }
};

export default {
    install: (app: App<Element>, options: any) => {
        options.init = initClickable(options);
        app.provide('clickablefield', options);
    }
}; 