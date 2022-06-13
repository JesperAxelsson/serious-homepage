import React, { useEffect, useState } from 'react';

import { PlusOutlined } from '@ant-design/icons';
import { Modal, Upload, Layout, Menu } from 'antd';
import type { RcFile, UploadProps } from 'antd/es/upload';
import type { UploadFile } from 'antd/es/upload/interface';
import { IAlbum } from './models/Album';

import { loadAlbumListFunc } from './GalleryApi';
import { NavLink, Outlet, Route, Routes } from 'react-router-dom';


const { Content, Sider } = Layout;

function Gallery2() {
    const [previewVisible, setPreviewVisible] = useState(false);
    const [previewImage, setPreviewImage] = useState('');
    const [previewTitle, setPreviewTitle] = useState('');
    const [fileList, setFileList] = useState<UploadFile[]>([
        {
            uid: '-1',
            name: 'image.png',
            status: 'done',
            url: 'https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png',
        },
        {
            uid: '-2',
            name: 'image.png',
            status: 'done',
            url: 'https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png',
        },
        {
            uid: '-3',
            name: 'image.png',
            status: 'done',
            url: 'https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png',
        },
        {
            uid: '-4',
            name: 'image.png',
            status: 'done',
            url: 'https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png',
        },
        {
            uid: '-xxx',
            percent: 50,
            name: 'image.png',
            status: 'uploading',
            url: 'https://zos.alipayobjects.com/rmsportal/jkjgkEfvpUPVyRjUImniVslZfWPnJuuZ.png',
        },
        {
            uid: '-5',
            name: 'image.png',
            status: 'error',
        },
    ]);

    const handleChange: UploadProps['onChange'] = ({ fileList: newFileList }) =>
        setFileList(newFileList);

    const uploadButton = (
        <div>
            <PlusOutlined />
            <div style={{ marginTop: 8 }}>Upload</div>
        </div>
    );


    const getBase64 = (file: RcFile): Promise<string> =>
        new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.readAsDataURL(file);
            reader.onload = () => resolve(reader.result as string);
            reader.onerror = error => reject(error);
        });

    const handlePreview = async (file: UploadFile) => {
        if (!file.url && !file.preview) {
            file.preview = await getBase64(file.originFileObj as RcFile);
        }

        setPreviewImage(file.url || (file.preview as string));
        setPreviewVisible(true);
        setPreviewTitle(file.name || file.url!.substring(file.url!.lastIndexOf('/') + 1));
    };

    const handleCancel = () => setPreviewVisible(false);

    return (
        <div>
            Gallery change things
            <>
                <Upload
                    action="https://www.mocky.io/v2/5cc8019d300000980a055e76"
                    listType="picture-card"
                    fileList={fileList}
                    onPreview={handlePreview}
                    onChange={handleChange}
                >
                    {uploadButton}
                </Upload>

                <Modal visible={previewVisible} title={previewTitle} footer={null} onCancel={handleCancel}>
                    <img alt="example" style={{ width: '100%' }} src={previewImage} />
                </Modal>
            </>
        </div>
    )
}

function GalleryEdit (){
    return <div>Dood</div>
}

function Gallery() {
    const [error, setError] = useState(null as any);
    const [isLoaded, setIsLoaded] = useState(false);
    const [albums, setAlbums] = useState([] as IAlbum[]);

    const loadRecipies = loadAlbumListFunc(setIsLoaded, setAlbums, setError);

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

                        {albums.map(item => (
                            <Menu.Item key={item.id}>
                                <NavLink to={item.id + ''}> {item.title} </NavLink>
                            </Menu.Item>
                        ))}

                    </Menu>
                </Sider>

                <Content style={{ padding: '0 10px' }}>
                    <Routes>
                        <Route path=":id" element={<GalleryEdit />} />
                        <Route path="create" element={<GalleryEdit />} />
                        <Route path="*" element={<div>Invalid route </div>} />
                    </Routes>

                    <Outlet />

                </Content>
            </Layout>
        )
    }

}
export default Gallery;
