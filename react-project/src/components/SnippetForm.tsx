import * as yup from 'yup';
import Col from 'react-bootstrap/Col';
import icons from '../icons';
import { Button, Form } from 'react-bootstrap';
import { Formik } from 'formik';
import { Snippet } from '../client/snippets';
import { fromSimpleDate, toSimpleDate } from '../dateUtils';
import { useState } from 'react';

export interface IEditSnippetFormProps {
  title: string,
  snippet: Snippet,
  onSubmit: (values: ISnippetFormValues) => void,
}

const schema = yup.object().shape({
  title: yup.string().required(),
  sharedBy: yup.string().required(),
  href: yup.string().url().required(),
  sharedOn: yup.date().required(),
  hidden: yup.boolean().required(),
  summary: yup.string().required(),
  description: yup.string(),
  icon: yup.string().required().oneOf(Object.values(icons)),
});

export interface ISnippetFormValues {
  title: string,
  sharedBy: string,
  href: string,
  sharedOn: Date,
  hidden: boolean,
  summary: string,
  description: string,
  icon: string,
}

export default function SnippetForm(props: IEditSnippetFormProps) {
  let snippet = useState(props.snippet)[0];

  return <Formik
    initialValues={{ ...snippet, sharedOn: toSimpleDate(snippet.sharedOn) }}
    validationSchema={schema}
    onSubmit={(values) => props.onSubmit({ ...values, sharedOn: fromSimpleDate(values.sharedOn), })}>
    {({
      handleSubmit,
      handleChange,
      handleBlur,
      values,
      touched,
      isValid,
      errors,
    }) => (
      <Form onSubmit={handleSubmit} action="#form">
        <Form.Row>
          <Col>
            <h2>{props.title}</h2>
          </Col>
        </Form.Row>
        <Form.Row>
          <Form.Group as={Col} controlId="title">
            <Form.Label>Title</Form.Label>
            <Form.Control
              type="text"
              name="title"
              value={values.title}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.title && !!errors.title} />
            <Form.Text>
              The title of the link - the blue part that you click on.
            </Form.Text>
            <Form.Control.Feedback type="invalid">
              {errors.title}
            </Form.Control.Feedback>
          </Form.Group>
          <Form.Group as={Col} controlId="sharedBy">
            <Form.Label>Shared by</Form.Label>
            <Form.Control
              type="text"
              name="sharedBy"
              value={values.sharedBy}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.sharedBy && !!errors.sharedBy} />
            <Form.Text>
              Who shared this with the community? Prefer Discord names to real
              names.
            </Form.Text>
            <Form.Control.Feedback type="invalid">
              {errors.sharedBy}
            </Form.Control.Feedback>
          </Form.Group>
        </Form.Row>
        <Form.Row>
          <Form.Group as={Col} controlId="href">
            <Form.Label>URL</Form.Label>
            <Form.Control
              type="text"
              name="href"
              value={values.href}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.href && !!errors.href} />
            <Form.Text>
              The URL to whatever interesting thing this snippet is about.
            </Form.Text>
            <Form.Control.Feedback type="invalid">
              {errors.href}
            </Form.Control.Feedback>
          </Form.Group>
          <Form.Group as={Col} controlId="sharedOn">
            <Form.Label>Shared on</Form.Label>
            <Form.Control
              type="text"
              name="sharedOn"
              value={values.sharedOn}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.sharedOn && !!errors.sharedOn} />
            <Form.Text>
              When was this shared, in ISO-8601 date format please.
            </Form.Text>
            <Form.Control.Feedback type="invalid">
              {errors.sharedOn}
            </Form.Control.Feedback>
          </Form.Group>
        </Form.Row>
        <Form.Row>
          <Form.Group as={Col} controlId="icon">
            <Form.Label>Icon</Form.Label>
            <Form.Control
              as="select"
              name="icon"
              value={values.icon}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.icon && !!errors.icon}>
              {Object.entries(icons).map(([iconName, iconFile]) => {
                return <option
                  key={iconFile}
                  value={iconFile}>
                  {iconName}
                </option>;
              })}
            </Form.Control>
            <Form.Text>
              Pick the icon used for this link. Adding icons requires a
              code-change to the website, see <a
                href="https://github.com/idevgames/idevgames.com/blob/mainline/react-project/src/icons.ts">
                <code>icons.ts</code>
              </a>.
            </Form.Text>
          </Form.Group>
          <Form.Group as={Col} controlId="summary">
            <Form.Label>Summary</Form.Label>
            <Form.Control
              type="textarea"
              name="summary"
              value={values.summary}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.summary && !!errors.summary} />
            <Form.Text>
              Describe this snippet in about a sentence, maybe two.
            </Form.Text>
            <Form.Control.Feedback type="invalid">
              {errors.summary}
            </Form.Control.Feedback>
          </Form.Group>
        </Form.Row>
        <Form.Row>
          <Form.Group as={Col} controlId="description">
            <Form.Label>Description</Form.Label>
            <Form.Control
              type="textarea"
              name="description"
              value={values.description}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={touched.description && !!errors.description} />
            <Form.Text>
              Describe this snippet, in about a paragraph. This isn't used on
              the site currently, so leaving it blank is a-ok.
            </Form.Text>
            <Form.Control.Feedback type="invalid">
              {errors.description}
            </Form.Control.Feedback>
          </Form.Group>
        </Form.Row>
        <Form.Row>
          <Form.Group as={Col} controlId="hidden">
            <Form.Check
              name="hidden"
              label="Hidden"
              checked={values.hidden}
              onChange={handleChange}
              onBlur={handleBlur}
              isInvalid={!!errors.hidden}
              feedback={errors.hidden}
              id="hidden" />
          </Form.Group>
          <Form.Group as={Col} controlId="submit">
            <div style={{ float: 'right', marginBottom: 0 + 'px' }}>
              <Button type="submit" disabled={!isValid}>Save</Button>
            </div>
          </Form.Group>
        </Form.Row>
      </Form>)}
  </Formik>;
}
