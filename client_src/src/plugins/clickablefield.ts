import { toValue, type App } from "vue";

type OnSaveAction = (form: any) => void;
type OnValidate = (value: string) => boolean;

type FieldValidator = {
    [key: string]: OnValidate
};


const getInputElement = (el: Element, options: any): Element => {
    const name: string = el.getAttribute('data-name') || '', 
        elType: string = el.getAttribute('data-type')?.toLowerCase() || 'text',
        inputDefaultCssClass = options.inputDefaultCssClass || 'form-control',
        errorCssClass = options.errorCssClass || 'error-message';

    let els = document.getElementsByName(name); 
    let inputEl: Element | null = els.length == 1 ? els[0] : null; 
    const wasNull = inputEl === null; 


    switch(elType) {
        case 'text': 
            inputEl = inputEl === null ? document.createElement('input') : inputEl; 
            inputEl.setAttribute('type', 'text'); 
        break;
        case 'number':  
            inputEl = inputEl === null ? document.createElement('input') : inputEl;
            inputEl.setAttribute('type', 'number'); 
        break;
        case 'textarea':
            inputEl = inputEl === null ? document.createElement('textarea') : inputEl; 
        break;
        default: 
            throw Error('Unsupported type ' + elType);
    }

    inputEl.setAttribute('name', name);
    inputEl.classList.add(inputDefaultCssClass);
    inputEl.classList.add('hide-field');

    if(wasNull) {
        el.appendChild(inputEl);
    }

    return inputEl;
}


const initClickable = (options: any) => {
    const cssClass = options.cssClass || 'clickable-field';

    return (form: any, sourceValues: any, onSave: OnSaveAction, validators?: FieldValidator) => {

        const fields: Element[] = Array.from(document.getElementsByClassName(cssClass));

        fields.forEach((item: Element) => {

            const clickableField = (e: any) => {
                const element = e.target, 
                    input = <any>getInputElement(element, options), 
                    dataName = input.name;

                if (element.classList.contains(cssClass)) {
                    input.setAttribute('name', dataName);
                    const validator = validators ? validators[dataName] : null;
                    input.value = sourceValues !== null ? sourceValues[dataName] : '';

                    input.addEventListener('blur', () => {
                        if(!validator || validator && validator(input.value)) {
                            onSave(form);
                            sourceValues[input.name] = input.value;
                            item.innerHTML = input.value;
                        }
                    })

                    const firstNode = element.childNodes[0];
                    if (firstNode !== null && firstNode !== undefined) {
                        element.removeChild(firstNode);
                    }

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