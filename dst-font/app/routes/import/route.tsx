import { json, unstable_composeUploadHandlers, unstable_createFileUploadHandler, unstable_createMemoryUploadHandler, unstable_parseMultipartFormData, writeAsyncIterableToWritable, type LoaderFunctionArgs } from "@remix-run/node";
import { Form, useFetcher, useLoaderData, useSubmit } from "@remix-run/react";
import { useTranslation } from "react-i18next";
import ky from 'ky';



let upload_file = {
  file_name: '',

}

export async function action({ request }: LoaderFunctionArgs) {

  const formdata = await request.formData();
  let _file_blob: any = formdata.get('resume');

  const _formdata = new FormData();
  _formdata.append('file', _file_blob);
  let _json = null;
  try {
    _json = await ky.post('http://localhost:8080/save_files', { body: _formdata }).json();
  } catch (error: any) {
    console.log(error);
    if (error.name === 'HTTPError') {
      _json = await error.response.text();
    }
  }

  upload_file.file_name = _file_blob.name
  return _json;
}



export async function loader({ request }: LoaderFunctionArgs) {
  return json({ upload_file });
}


export default function _import() {

  const { upload_file } = useLoaderData<typeof loader>();

  const { t } = useTranslation();


  const submit = useSubmit();
  const fetcher = useFetcher();

  return (
    <>
      <div className="flex-auto self-center content-center"
        style={{ padding: '20px', overflow: 'auto' }}
      >
        <fetcher.Form method="post" encType="multipart/form-data"
          onChange={(e) => { if (e.currentTarget) submit(e.currentTarget) }}>
          <div className="file is-primary has-name is-boxed">
            <label className="file-label">
              <input className="file-input" type="file" name="resume" />
              <span className="file-cta">
                <span className="file-icon">
                  <i className="fas fa-upload"></i>
                </span>
                <span className="file-label"> {t('import.choose_file')} </span>
              </span>
              <span className="file-name"> {upload_file.file_name} </span>
            </label>
          </div>
        </fetcher.Form>
        <div>success</div>
      </div>
    </>
  );
}