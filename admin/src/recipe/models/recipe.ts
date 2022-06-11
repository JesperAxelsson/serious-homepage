

interface IRecipe {
    id: number,
    title: string,
    description: string,
    content: string,
}

enum RecipeAction {
    Title,
    Description,
    Content,
    Reset,
}

interface IRecipeReducerAction {
    type: RecipeAction,
    value: any,
}

function recipeReducer(state: IRecipe, action: IRecipeReducerAction): IRecipe {
    switch (action.type) {
        case RecipeAction.Title:
            return { ...state, title: action.value }
        case RecipeAction.Description:
            return { ...state, description: action.value }
        case RecipeAction.Content:
            return { ...state, content: action.value }
        case RecipeAction.Reset:
            return { ...action.value }
        default:
            throw new Error();
    }
}

export type { IRecipe, IRecipeReducerAction };
export { RecipeAction, recipeReducer };
