import React from "react";
import {Checkbox, IconButton, Tooltip} from "@mui/material";
import {
    Autorenew,
    FlipCameraIos,
    GridOn,
    LineAxis,
    Pentagon,
    PentagonOutlined, Tune
} from "@mui/icons-material";

export function CheckAutoRotate(props) {
    return (
        <Tooltip title="Toggle Auto-Rotation">
            <Checkbox defaultChecked={true} onChange={props.onChange} icon={<Autorenew />} checkedIcon={<Autorenew color="primary" />} />
        </Tooltip>
    )
}

export function CheckAxes(props) {
    return (
        <Tooltip title="Toggle Axes">
            <Checkbox defaultChecked={false} onChange={props.onChange} icon={<LineAxis />} checkedIcon={<LineAxis color="primary" />} />
        </Tooltip>
    )
}

export function CheckGrid(props) {
    return (
        <Tooltip title="Toggle Grid">
            <Checkbox defaultChecked={false} onChange={props.onChange} icon={<GridOn />} checkedIcon={<GridOn color="primary" />} />
        </Tooltip>
    )
}

export function CheckWireframe(props) {
    return (
        <Tooltip title="Toggle Wireframe">
            <Checkbox defaultChecked={false} onChange={props.onChange} icon={<PentagonOutlined />} checkedIcon={<Pentagon color="primary" />} />
        </Tooltip>
    )
}

export function ButtonResetCamera(props) {
    return (
        <Tooltip title="Reset Camera Position">
            <IconButton onClick={props.onClick} color="primary">
                <FlipCameraIos />
            </IconButton>
        </Tooltip>
    )
}

export function ButtonResetParameters(props) {
    return (
        <Tooltip title="Reset Model Parameters">
            <IconButton onClick={props.onClick} color="primary">
                <Tune />
            </IconButton>
        </Tooltip>

    )
}
