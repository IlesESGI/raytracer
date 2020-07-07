# Ray tracer Rust 4IABD
Projet pour le cours de Rust par le groupe composé de Hedi BOUFADEN, Kamel MALKI et Ilès BENKOUSSA

Pour éxectuer le programme :

    cd app
    rustup run nightly cargo run --release <chemin du json définissant la scene> <nom de l'image produite> <booleen multithreading>

    Exemple : rustup run nightly cargo run --release scenes/objects.json image.ppm true

Ce programme génère une image au format PPM de ce fait elle modifiable avec la lib "libppm" du semestre précedent. Elle se trouve dans le dossier /app.

## Benchmarking

Il est possible de benchmarker le programme, la version multithreadée et la version monothreadée, cependant comme l'opération pour le rendu peut-être très longue, nous générons une image de 9x10 pixels sinon le benchmark dure beaucoup trop longtemps. Cette taille très petite fait que la version monothread est plus rapide que la multithread et n'est pas représentative des cas réels avec une dimension raisonable.

On se rend compte avec une image de dimension 1920*1080 que le temps moyen est de 12 secondes en multithread contre 20 secondes en monothread sur un processeur Intel(R) Core(TM) i5-7600K CPU @ 3.80GHz avec 16 Go de RAM.
La version multithreadée fonctionne donc correctement !

Pour lancer le benchmark :

    cd app
    rustup run nightly cargo bench