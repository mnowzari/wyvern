# wyvern
A command-line image processing tool written in Rust.

This tool has been tested with .png and .jpg files. Support for other formats has not been verified yet!

### Features:
- Image downscaling (50% downscaling only for now)
- Batch downscaling of a directory of images
- Fast approximate k-means clustering to get the 'common colors' of an image, output as a new image
- Edge detection for a given image
- Pixel sorting with an option to select the 'direction' of the sorted pixels
- Denoise an image using nearest-neighbor averages

### Examples

#### Downscale a single image
The new downscaled image will be saved under _\path\to\image_downscaled_

    wyvern image-downscale "\path\to\image"

#### Batch downscale a directory of images
The output of this command will be saved under a new directory _\path\to\directory\resized_images_

    wyvern batch-resize "\path\to\directory" png
    
#### Return set of common colors in an image
The output is saved under _\path\to\image_common_colors_

    wyvern common-colors "\path\to\image"

#### Generate new image highlighting the edges of an image
The new image will be saved under _\path\to\image_edges_

    wyvern edge-detect "\path\to\image"

#### Pixel sort an image
This will pixel sort and save the output under _\path\to\image_pixelsorted_

    wyvern pixel-sort "path\to\image"

#### Denoise an image
This will denoise a given image and save the denoised output under _\path\to\image_denoised_

    wyvern denoise "path\to\image"

As always, use the _--help_ flag to learn more about each command and the particular flags/args you can provide.