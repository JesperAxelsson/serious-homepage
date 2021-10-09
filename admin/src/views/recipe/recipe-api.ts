import { createAxios } from '@/endpoint'
import { Axios, AxiosResponse } from 'axios';


export default interface IRecipe {
    id: number;
    title: string;
    description: string;
    content: string;
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

    // saveRecipe() { }

    public async update(recipe: IRecipe): Promise<AxiosResponse<IRecipe>> {
        const resp = await this.axios.put('recipe', recipe);
        console.log("Seving recipe", recipe, resp)
        return resp;
    }

    public async create(recipe: IRecipe): Promise<AxiosResponse<IRecipe>> {
        const resp = await this.axios.post('recipe', recipe);
        return resp;
    }
}
