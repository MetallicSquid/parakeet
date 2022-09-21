import './GalleryView.css';

import React from 'react';
import {
    Card,
    CardActionArea,
    CardHeader,
    CardMedia,
    CardContent,
    Typography,
    Grid
} from '@mui/material';
import { Link } from 'react-router-dom';

function ModelCard(props) {
    return (
        <Card sx={{ width: 300 }} className={props.id}>
            <CardActionArea component={Link} to={'/' + props.id}>
                <CardHeader
                    title={props.name}
                    subheader={props.date}
                />
                <CardMedia
                    component="img"
                    height="200"
                    image={props.image_path}
                />
                <CardContent>
                    <Typography>
                        {props.description}
                    </Typography>
                </CardContent>
            </CardActionArea>
        </Card>
    );
}

function Gallery(props) {
    return (
        <>
            <div className="Gallery-div">
                <Grid
                    container
                    spacing={{ xs: 2, md: 3 }}
                    columns={{ xs: 4, sm: 8, md: 12 }}
                    justifyContent="center"
                >
                    {props.models.map(model => (
                        <Grid item>
                            <ModelCard
                                id={model.model_id}
                                name={model.name}
                                date={model.creation_date}
                                image_path={model.image_path}
                                description={model.description}
                            />
                        </Grid>
                    ))}
                </Grid>
            </div>
        </>
   );
}

class GalleryView extends React.Component {
    gatherSiteInfo() {
        return {}
    }

    render() {
        const author = this.gatherSiteInfo();
        return (
            <div className="GalleryView-div">
                <div className="Title-div">
                    <h1 className="Title-heading">🦜 Guillaume's Parakeet 🦜</h1>
                </div>
                <Gallery models={this.props.models}/>
            </div>
        );
    }
}

export default GalleryView;
