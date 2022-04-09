import './GalleryView.css';
import React from 'react';
import Card from '@mui/material/Card';
import CardActionArea from '@mui/material/CardActionArea';
import CardHeader from '@mui/material/CardHeader';
import CardMedia from '@mui/material/CardMedia';
import CardContent from '@mui/material/CardContent';
import Typography from "@mui/material/Typography";
import Grid from '@mui/material/Grid';
import Box from "@mui/material/Box";

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
        return [
            {
                id: "000001",
                name: "Test Model",
                date: "April 9, 2022",
                image_path: "",
                image_alt: "Test Model",
                description: "A description for the test model."
            },
            {
                id: "000002",
                name: "Test Model",
                date: "April 9, 2022",
                image_path: "",
                image_alt: "Test Model",
                description: "A description for the test model."
            },
            {
                id: "000003",
                name: "Test Model",
                date: "April 9, 2022",
                image_path: "",
                image_alt: "Test Model",
                description: "A description for the test model."
            },
            {
                id: "000004",
                name: "Test Model",
                date: "April 9, 2022",
                image_path: "",
                image_alt: "Test Model",
                description: "A description for the test model."
            },
            {
                id: "000005",
                name: "Test Model",
                date: "April 9, 2022",
                image_path: "",
                image_alt: "Test Model",
                description: "A description for the test model."
            }
        ]
    }

    handleCardClick(id) {

    }

    render() {
        const models = this.gatherModelInfo();

        return (
            <React.Fragment>
                <div className="Gallery-div">
                    <Box>
                        <Grid container spacing={4}>
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
                    </Box>
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
