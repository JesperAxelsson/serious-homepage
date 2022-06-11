import React, { useEffect, useState } from 'react';
import IRecipe from './models/recipe';

import 'react-quill/dist/quill.snow.css';
import { Layout, Menu } from 'antd';
import { NavLink, Outlet, Route, Routes } from 'react-router-dom';
import _ from 'lodash';
import { EditRecipe, CreateRecipe } from './EditRecipe';


const { Header, Content, Sider } = Layout;

function Recipe() {
    const [value, setValue] = useState('');
    const [error, setError] = useState(null as any);
    const [isLoaded, setIsLoaded] = useState(false);
    const [recipies, setRecipies] = useState([] as IRecipe[]);

    useEffect(() => {
        fetch('http://localhost:3030/recipe')
            .then(res => res.json()).then(
                (result) => {
                    setIsLoaded(true);
                    console.log(result);
                    setRecipies(result);
                },
                // Note: it's important to handle errors here
                // instead of a catch() block so that we don't swallow
                // exceptions from actual bugs in components.
                (error) => {
                    setIsLoaded(true);
                    setError(error);
                })
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
                        {/* <Route path="" element={<ContentWrapper recipe={recipies[0]} />}> */}
                        <Route path=":id" element={<EditRecipe recipe={_.find(recipies, { id: 5 })} />} />
                        <Route path="create" element={<CreateRecipe />} />
                        <Route path="*" element={<div>Invalid route </div>} />
                        {/* </Route> */}
                    </Routes>

                    <Outlet />


                </Content>
            </Layout>
        )
    }

}

export default Recipe;
