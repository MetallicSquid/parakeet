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

function Title(author) {
    return (
        <div className="Title-div">
            <h1 className="Title-heading">🦜 {author}'s Parakeet 🦜</h1>
        </div>
    );
}

function ModelCard(props) {
    return (
        <Card sx={{ width: 354 }} className={props.id}>
            <CardActionArea onClick={props.onClick}>
                <CardHeader
                    title={props.name}
                    subheader={props.date}
                />
                <CardMedia
                    component="img"
                    height="200"
                    image={props.image_path}
                    alt={props.image_alt}
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

class Gallery extends React.Component {
    gatherModelInfo() {

    }

    handleCardClick(id) {

    }

    render() {
        const models = this.gatherModelInfo();

        return (
            <React.Fragment>
                <div className="Gallery-div">
                    <Grid
                        container
                        spacing={{ xs: 2, md: 3 }}
                        columns={{ xs: 4, sm: 8, md: 12 }}
                        justifyContent="center"
                        paddingLeft="50px"
                        paddingRight="50px"
                    >
                        {models.map(model => (
                            <Grid item>
                                <ModelCard
                                    key={model.id}
                                    name={model.name}
                                    date={model.date}
                                    image_path={model.image_path}
                                    image_alt={model.image_alt}
                                    description={model.description}
                                    onClick={() => this.handleCardClick(model.id)}
                                />
                            </Grid>
                        ))}
                    </Grid>
                </div>
            </React.Fragment>
        );
    }
}

class GalleryView extends React.Component {
    gatherSiteInfo() {
        return {}
    }

    render() {
        const author = this.gatherSiteInfo();
        return (
            <div className="GalleryView-div">
                {Title("Guillaume")}
                <Gallery />
            </div>
        );
    }
}

export default GalleryView;
