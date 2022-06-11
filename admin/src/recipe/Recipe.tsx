import { useEffect, useState, Dispatch, SetStateAction } from 'react';
import { IRecipe } from './models/recipe';

import 'react-quill/dist/quill.snow.css';
import { Layout, Menu } from 'antd';
import { NavLink, Outlet, Route, Routes } from 'react-router-dom';
import { EditRecipe, CreateRecipe } from './EditRecipe';

/// Todo:
/// - Reload menu when recipe edited or created
/// - Clean up layout of buttons
/// - Make it responsive

const { Content, Sider } = Layout;

function loadRecipiesFunc(
    setIsLoaded: Dispatch<SetStateAction<boolean>>,
    setRecipies: Dispatch<SetStateAction<IRecipe[]>>,
    setError: Dispatch<SetStateAction<any>>): () => void {

    return () => {
        fetch('http://localhost:3030/recipe')
            .then(res => res.json()).then(
                (result) => {
                    setIsLoaded(true);
                    console.log("Recipies loaded: ", result);
                    setRecipies(result);
                },
                // Note: it's important to handle errors here
                // instead of a catch() block so that we don't swallow
                // exceptions from actual bugs in components.
                (error) => {
                    setIsLoaded(true);
                    setError(error);
                })
    }
}

function Recipe() {
    const [error, setError] = useState(null as any);
    const [isLoaded, setIsLoaded] = useState(false);
    const [recipies, setRecipies] = useState([] as IRecipe[]);

    const loadRecipies = loadRecipiesFunc(setIsLoaded, setRecipies, setError);

    useEffect(() => {
        loadRecipies();
    }, []);

    if (error) {
        return <div>Error: {error.message}</div>;
    } else if (!isLoaded) {
        return <div>Loading...</div>;
        // } else if (recipies.length == 0) {
        //     return <div>No recipies yet</div>;
    } else {
        return (
            <Layout >
                <Sider theme='light'>
                    <Menu>
                        <Menu.Item key='-1'>
                            <NavLink to='create'> Create new </NavLink>
                        </Menu.Item>

                        {recipies.map(item => (
                            <Menu.Item key={item.id}>
                                <NavLink to={item.id + ''}> {item.title} </NavLink>
                            </Menu.Item>
                        ))}

                    </Menu>
                </Sider>

                <Content style={{ padding: '0 10px' }}>
                    <Routes>
                        <Route path=":id" element={<EditRecipe recipies={recipies} />} />
                        <Route path="create" element={<CreateRecipe />} />
                        <Route path="*" element={<div>Invalid route </div>} />
                    </Routes>

                    <Outlet />

                </Content>
            </Layout>
        )
    }

}

export default Recipe;
