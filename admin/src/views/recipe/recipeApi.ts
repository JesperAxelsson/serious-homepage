import { createAxios } from '@/endpoint'
import { Axios } from 'axios';


export default interface IRecipe {
    id: number;
    title: string;
    description: string;
}

export default class RecipeApi {
    axios: Axios;

    constructor() {
        this.axios = createAxios();
    }

    public async getAll(): Promise<IRecipe[]> {
        const response = await this.axios.get('recipe');
        console.log("Got response ", response);
        return response.data
    }

    // getContent() {

    // }

    // saveRecipe() { }

    // saveContent() { }
}
