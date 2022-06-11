import React, { useEffect, useReducer, useState } from 'react';
import IRecipe from './models/recipe';

import ReactQuill from 'react-quill';
import 'react-quill/dist/quill.snow.css';
import { Col, Layout, Menu, Row, Input, Divider, Affix, Button } from 'antd';
import { NavLink, Outlet, Route, Routes } from 'react-router-dom';
import _ from 'lodash';
import Title from 'antd/lib/typography/Title';


const { Header, Content, Sider } = Layout;

enum RecipeAction {
    title,
    description,
    content,
}
interface IRecipeReducerAction {
    type: RecipeAction,
    value: string,
}

function recipeReducer(state: IRecipe, action: IRecipeReducerAction): IRecipe {
    switch (action.type) {
        case RecipeAction.title:
            return { ...state, title: action.value }
        case RecipeAction.description:
            return { ...state, description: action.value }
        case RecipeAction.content:
            return { ...state, content: action.value }
        default:
            throw new Error();
    }
}

interface IContentProps {
    recipe: IRecipe,
}

function EditRecipe(props: IContentProps | undefined = undefined) {
    const recipe = props?.recipe ?? {
        id: -1,
        title: "",
        description: '',
        content: '',
    }

    const createNew = props?.recipe === undefined;

    const [state, dispatch] = useReducer(recipeReducer, recipe);
    // const [title, setTitle] = useState(recipe.title);
    // const [description, setDescription] = useState(recipe.description);
    // const [content, setContent] = useState(recipe.content);

    // useEffect(() => {
    //     setTitle(recipe.title)
    //     setDescription(recipe.description)
    //     setContent(recipe.content)
    // }
    //     , []);
    console.log("Props", props, state.title, recipe)

    return (
        <div >
            <Row >
                <Col span={23}>
                    {/* <Divider orientation='left' > */}
                    <Title level={3}>
                        {
                            createNew ? 'CreateNew' : 'Edit ' + props.recipe.title
                        }
                    </Title>
                    {/* </Divider> */}
                </Col>
                <Col span={1}>
                    {
                        createNew ?
                            <Button type="primary">
                                Create
                            </Button> :
                            <Button type="primary" onClick={() => saveRecipe(state)}>
                                Save
                            </Button>
                    }

                </Col>
            </Row>
            <Row>
                <Col span={12} >
                    <Title level={4}>{state.title}<br></br> <sub>{state.description}</sub></Title>
                    <div dangerouslySetInnerHTML={{ __html: state.content }} />
                </Col>

                <Col span={12} >
                    <Input placeholder="Title" value={state.title} onChange={(e) => dispatch({ type: RecipeAction.title, value: e.target.value })} />
                    <Input placeholder="Description" value={state.description} onChange={(e) => dispatch({ type: RecipeAction.description, value: e.target.value })} />
                    <ReactQuill theme='snow' value={state.content} onChange={value => dispatch({ type: RecipeAction.content, value: value })} />
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
            if (response.status == 200) {
                console.log('Success');

            } else {
                console.log('Error, save failed:', response);
            }
        })
        .catch((error) => {
            console.error('Error:', error);
        });
}

EditRecipe.defaultProps = {
    recipe: undefined,
}

function CreateRecipe() {
    return EditRecipe(undefined);
}


export   { EditRecipe, CreateRecipe };
// export { EditRecipe, CreateRecipe };
