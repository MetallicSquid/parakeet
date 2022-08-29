import React from "react";
import {Checkbox, IconButton, Tooltip} from "@mui/material";
import {Autorenew, FlipCameraIos, GridOn, LineAxis, PentagonOutlined, SquareRounded} from "@mui/icons-material";

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
            <Checkbox defaultChecked={false} onChange={props.onChange} icon={<PentagonOutlined />} checkedIcon={<PentagonOutlined color="primary" />} />
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
