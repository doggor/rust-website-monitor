import React, { useState, useEffect, useCallback, useMemo } from "react";
import Container from '@material-ui/core/Container';
import { makeStyles } from '@material-ui/core/styles';
import MaterialTable from "material-table";
import { listSites, createSite, updateSite, removeSite } from "./services";

const tableColumns = [
    { title: "#ID", field: "id", editable: "never" },
    { title: "Domain", field: "domain" },
    { title: "Cert Expired At", field: "cert_expired_at", type: "datetime", editable: "never" },
    {
        title: "Active", field: "active", lookup: { false: "Inactive", true: "Active" }, editable: "onUpdate", cellStyle: text => ({
            color: text === "Active" ? "#4caf50" : "#f44336",
        })
    },
];

const tableOptions = {
    actionsColumnIndex: tableColumns.length,
    pageSize: 10,
    pageSizeOptions: [10, 20, 50, 100],
    headerStyle: {
        fontWeight: "bold",
    },
};

const useStyles = makeStyles(theme => ({
    container: {
        marginTop: theme.spacing(2),
        marginBottom: theme.spacing(2),
    },
}));

export default function App() {
    const classes = useStyles();

    const [data, setData] = useState([]);

    useEffect(() => {
        listSites().then(setData).catch(console.error);
    }, []);

    const onRowAdd = useCallback(newData => {
        return createSite(newData.domain).then(listSites).then(setData).catch(console.error);
    });

    const onRowUpdate = useCallback((newData, oldData) => {
        return updateSite(oldData.id, newData.domain, newData.active).then(listSites).then(setData).catch(console.error);
    });

    const onRowDelete = useCallback(oldData => {
        return removeSite(oldData.id).then(listSites).then(setData).catch(console.error);
    });

    const editable = useMemo(() => ({ onRowAdd, onRowUpdate, onRowDelete }), [onRowAdd, onRowUpdate, onRowDelete]);

    return (
        <Container maxWidth="md" className={classes.container}>
            <MaterialTable
                title="Domain List"
                columns={tableColumns}
                options={tableOptions}
                data={data}
                editable={editable}
            ></MaterialTable>
        </Container>
    );
}
