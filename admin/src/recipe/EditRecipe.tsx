import { useEffect, useReducer, useState } from 'react';
import { IRecipe, RecipeAction, recipeReducer } from './models/recipe';

import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import { Col, Row, Input, Button } from 'antd';
import { useParams } from 'react-router-dom';
import _ from 'lodash';
import Title from 'antd/lib/typography/Title';
import ICreateRecipe from './models/createRecipe';


interface IContentProps {
    recipies: IRecipe[],
}



function CreateRecipe() {
    const [recipe, setRecipe] = useState({
        id: -1,
        title: "",
        description: '',
        content: '',
    } as IRecipe);

    return InternalRecipe(recipe);
}


function EditRecipe(props: IContentProps) {
    const recipeParams: any = useParams();
    const [recipe, setRecipe] = useState({
        id: -1,
        title: "",
        description: '',
        content: '',
    } as IRecipe);

    useEffect(() => {
        let rec = _.find(props.recipies, { id: Number(recipeParams.id) })
            ?? _.first(props.recipies)
            ?? {
            id: -1,
            title: "",
            description: '',
            content: '',
        };

        setRecipe(rec as IRecipe);

    }, [recipeParams]);

    return InternalRecipe(recipe);
}

function InternalRecipe(param_recipe: IRecipe) {
    const createNew = param_recipe.id < 0;

    const [state, dispatch] = useReducer(recipeReducer, param_recipe);

    useEffect(() => {
        dispatch({ type: RecipeAction.Reset, value: param_recipe })
    }, [param_recipe]);

    return (
        <div >
            <Row >
                <Col span={23}>
                    <Title level={3}>
                        {
                            createNew ? 'Create' : 'Edit ' + param_recipe.title
                        }
                    </Title>
                </Col>
                <Col span={1}>
                    {
                        createNew ?
                            <Button type="primary" onClick={() => createRecipe(state)}>
                                Create
                            </Button> :
                            <div>
                                <Button type="primary" onClick={() => deleteRecipe(state)}>
                                    Delete
                                </Button>
                                <Button type="primary" onClick={() => saveRecipe(state)}>
                                    Save
                                </Button>
                            </div>
                    }

                </Col>
            </Row>
            <Row>
                <Col span={12} >
                    <Title level={4}>{state.title}<br></br> <sub>{state.description}</sub></Title>
                    <div dangerouslySetInnerHTML={{ __html: state.content }} />
                </Col>

                <Col span={12} >
                    <Input placeholder="Title" value={state.title} onChange={(e) => dispatch({ type: RecipeAction.Title, value: e.target.value })} />
                    <Input placeholder="Description" value={state.description} onChange={(e) => dispatch({ type: RecipeAction.Description, value: e.target.value })} />
                    <ReactQuill theme='snow' value={state.content} onChange={value => dispatch({ type: RecipeAction.Content, value: value })} />
                </Col>
            </Row>
        </div>
    );
}

function saveRecipe(recipe: IRecipe) {
    console.log("Save recipe: ", recipe);

    fetch('http://localhost:3030/recipe', {
        method: 'PUT', // or 'PUT'
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(recipe),
    })
        .then(response => {
            if (response.status === 200) {
                console.log('Success');

            } else {
                console.log('Error, save failed:', response);
            }
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}


function createRecipe(recipe: IRecipe) {
    const createRecipe = recipe as ICreateRecipe
    console.log("Create recipe: ", createRecipe);

    fetch('http://localhost:3030/recipe', {
        method: 'POST', // or 'PUT'
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(createRecipe),
    })
        .then(response => {
            if (response.status === 201) {
                console.log('Success');

            } else {
                console.log('Error, save failed:', response);
            }
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}

function deleteRecipe(recipe: IRecipe) {
    console.log("Delete recipe: ", recipe);

    fetch('http://localhost:3030/recipe/' + recipe.id, {
        method: 'DELETE', // or 'PUT'
    })
        .then(response => {
            if (response.status === 204) {
                console.log('Success');

            } else {
                console.log('Error, save failed:', response);
            }
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}

export { EditRecipe, CreateRecipe };
