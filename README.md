# Ray tracer Rust 4IABD
Projet pour le cours de Rust par le groupe composé de Hedi BOUFADEN, Kamel MALKI et Ilès BENKOUSSA

Pour éxectuer le programme :

    cd app
    rustup run nightly cargo run --release <chemin du json définissant la scene> <nom de l'image produite> <booleen multithreading>

    Exemple : rustup run nightly cargo run --release scenes/objects.json image.ppm true

Ce programme génère une image au format PPM de ce fait elle modifiable avec la lib "libppm" du semestre précedent. Elle se trouve dans le dossier /app.<br>
Guide suivi : https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

## Benchmarking

Il est possible de benchmarker le programme, la version multithreadée et la version monothreadée, cependant comme l'opération pour le rendu peut-être très longue, nous générons une image de 9x10 pixels sinon le benchmark dure beaucoup trop longtemps. Cette taille très petite fait que la version monothread est plus rapide que la multithread et n'est pas représentative des cas réels avec une dimension raisonable.

On se rend compte avec une image de dimension 1920*1080 que le temps moyen est de 12 secondes en multithread contre 20 secondes en monothread sur un processeur Intel(R) Core(TM) i5-7600K CPU @ 3.80GHz avec 16 Go de RAM.
La version multithreadée fonctionne donc correctement !

Pour lancer le benchmark :

    cd app
    rustup run nightly cargo bench

## Questions

### Is OpenGL use raytracing to render 3D scene? How does this work? What the benefits of both methods?

No OpenGL doesn't use raytracing. OpenGL uses rasterization.<br>
Rasterizing is widely used to render real-time 3D graphics such as games this is due to the way it balances the real-time performance needed with the ability to create the pretty pictures we've come to expect from modern games. Basically the way this works is the rasterizer looks at the thousands of triangles that make up the 3D scene and determines which will be visible in the current perspective. With that information, the engine then analyzes the light sources along with some other environmental details to add light and colour to the pixels on each triangle.<br>
So to conclude OpenGL, with rasterization, is way faster but less realistic and at the opposite, raytracing is slower but more realistic.
### What is shadering? How is it computed? Why smart people do raytracing in shaders?
A shader is a small program that runs on a graphics card or GPU which manipulates a 3D scene during the rendering pipeline before the image is drawn to the screen. These shaders allow for a range of different rendering effects to be created quickly enough for real-time applications such as games. There two types :
1. Vertex shader can manipulate the attributes of vertices which are the corner points of a polygon
2. Fragment shader is computing the colour of each pixel

Maybe smart people do raytracing in shaders because it is the best way to parallelize computing easily + it uses GPU hardware and take all profit  of it so they gain a lot of time.
