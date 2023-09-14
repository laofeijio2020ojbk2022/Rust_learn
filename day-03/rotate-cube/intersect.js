import * as THREE from './node_modules/three/build/three.module.js';

// 创建场景、摄像机和渲染器
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// 加载两个模型
const loader = new OBJLoader();
loader.load("path/to/geometry1.obj", (geometry1) => {
  const material1 = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
  const mesh1 = new THREE.Mesh(geometry1, material1);
  scene.add(mesh1);
});

loader.load("path/to/geometry2.obj", (geometry2) => {
  const material2 = new THREE.MeshBasicMaterial({ color: 0xff0000 });
  const mesh2 = new THREE.Mesh(geometry2, material2);
  scene.add(mesh2);

  // 执行交集操作
  const intersect = BooleanModifier.intersection(mesh1, mesh2);
  scene.add(intersect);
});

// 设置摄像机位置
camera.position.z = 5;

// 渲染场景 
