import { legacy_createStore as createStore } from 'redux';

const initialState = {
    text: ''
}

const reducer = (state = initialState, action) => {
    switch(action.type) {
        case 'SET_TEXT': return { ...state, text: action.payload }
        default: return state;
    }
}

const store = createStore(reducer);

export default store;