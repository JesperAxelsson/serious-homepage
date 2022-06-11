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
import Recipe from './recipe/Recipe'
import Gallery from './gallery/gallery'
import Title from 'antd/lib/typography/Title';

const { Header, Content, Footer, Sider } = Layout;


interface IMenuItem {
  label: string,
  key: string,
  link: string,
}

const menuItems: IMenuItem[] = [
  { label: 'Gallery', key: 'gallery', link: '/gallery' },
  { label: 'Recipe', key: 'recipe', link: '/recipe' },
  { label: 'Blog', key: 'blog', link: '/blog' },
];

function getMenuItem(key: string): IMenuItem | undefined {
  return _.find(menuItems, { key: key });
}

interface IContentProps {
  title: string,
}

function ContentWrapper(props: IContentProps) {
  return (
    <div>
      <Title level={2}>{props.title}</Title>

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
          onClick={props => { setMenu(props.key); navigate(getMenuItem(props.key)?.link ?? '/notfound') }}
          items={menuItems}
        />
      </Sider>

      <Content style={{ padding: '0 30px' }}>
        <Layout className="site-layout-background" style={{ padding: '24px 0' }}>

          <Content >
            <Routes>
              <Route path="/" element={<ContentWrapper title={getMenuItem(menu)?.label ?? ''} />}>
                <Route path="/blog" element={<Blog />} />
                <Route path="/recipe/*" element={<Recipe />} />
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
