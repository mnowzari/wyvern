# wyvern
A command-line image processing tool written in Rust.

This tool has been tested with .png and .jpg files. Support for other formats has not been verified yet!

_This tool is a work in progress._

#### Functions:
- Image downscaling (50% downscaling only for now)
- Batch downscaling of a directory of images
- Fast k-means clustering approximation to get the 'common colors' of an image
- Edge detection for a given image
- Pixel sorting with an option to select the 'direction' of the sorted pixels

#### Examples

##### Downscale a single image
The new downscaled image will be saved under _\path\to\image_downscaled_

    image-downscale "\path\to\image"

##### Batch downscale a directory of images
The output of this command will be saved under a new directory _\path\to\directory\resized_images_

    batch-resize "\path\to\directory" png
    
##### Return set of common colors in an image
The output is printed to stdout

    kmeans "\path\to\image"

##### Generate new image highlighting the edges of an image
The new image will be saved under _\path\to\image_edges_

    edge-detect "\path\to\image"

##### Pixel sort an image
This will pixel sort and save the output under _\path\to\image_pixelsorted_

    pixel-sort "path\to\image"

As always, use the _--help_ flag to learn more about each command and any particular flags/args you can provide.