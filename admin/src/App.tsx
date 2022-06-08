import React, { useState } from 'react';
// import logo from './logo.svg';
import './App.css';
import 'antd/dist/antd.css'; // or 'antd/dist/antd.less'
import { LaptopOutlined, NotificationOutlined, UserOutlined } from '@ant-design/icons';
import type { MenuProps } from 'antd';
import { Breadcrumb, Layout, Menu } from 'antd';

import { BrowserRouter as Router, Route, Link, Routes, Outlet } from "react-router-dom";
import { useNavigate } from "react-router-dom";

import _ from "lodash";

import Blog from './blog/blog'
import Recipe from './recipe/recipe'
import Gallery from './gallery/gallery'

const { Header, Content, Footer, Sider } = Layout;

const menuItems = [
  { label: 'Gallery', key: '/gallery', link: '/gallery' },
  { label: 'Recipe', key: '/recipe', link: '/recipe' },
  { label: 'Blog', key: '/blog', link: '/blog' },
];

interface IContentProps {
  title: String,
}

function ContentWrapper(props: IContentProps) {
  return (
    <div>
      <h1>{props.title}</h1>

      <Outlet />
    </div>
  );
}

function App() {
  const [collapsed, setCollapsed] = useState(false);
  const [menu, setMenu] = useState(menuItems[0].key);
  let navigate = useNavigate();

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider collapsible collapsed={collapsed} onCollapse={setCollapsed}>
        <div className="logo" />
        <Menu
          theme="dark"
          mode="inline"
          defaultSelectedKeys={[menu]}
          onClick={props => { setMenu(props.key); navigate(props.key) }}
          items={menuItems}
        />
      </Sider>

      <Content style={{ padding: '0 50px' }}>
        <Layout className="site-layout-background" style={{ padding: '24px 0' }}>

          <Content style={{ padding: '0 24px', minHeight: 280 }}>
            <Routes>
              <Route path="/" element={<ContentWrapper title={_.find(menuItems, { key: menu })?.label ?? ''} />}>
                <Route path="/blog" element={<Blog />} />
                <Route path="/recipe" element={<Recipe />} />
                <Route path="/gallery" element={<Gallery />} />
              </Route>
            </Routes>
          </Content>
        </Layout>
      </Content>

    </Layout>
  );
}

export default App;
